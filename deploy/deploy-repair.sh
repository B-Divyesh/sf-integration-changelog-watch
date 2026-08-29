#!/bin/bash
# Build and deploy the single-replica, durable topology for this product.
# Unlike the generic factory helper, this product may not use its 1–3 replica
# default: SQLite and the per-IP rate bucket have exactly one owner.
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
sha=$(git -C "$root" rev-parse HEAD)
tag="sf-integration-changelog-watch:${sha:0:12}"
subscription=${AZURE_SUBSCRIPTION_ID:-283af945-693b-4a6e-b952-df928d0a18a9}
resource_group=sociobot
app=sf-integration-changelog-watch
environment=factory-env
resource_id="/subscriptions/$subscription/resourceGroups/$resource_group"
base="https://management.azure.com$resource_id"
environment_id="$resource_id/providers/Microsoft.App/managedEnvironments/$environment"
identity_id="$resource_id/providers/Microsoft.ManagedIdentity/userAssignedIdentities/factory-worker-identity"
certificate_id="$environment_id/managedCertificates/cert-integration-changelog-watch"
site_url=${DEPLOY_SITE_URL:-https://integration-changelog-watch.sociobot.in}

# A build SHA is only a useful release identity when it names the exact source
# that reviewers can fetch. Refuse dirty or merely-local revisions before an
# image is built, then check the public deployment after the revision rolls.
if [ -n "$(git -C "$root" status --porcelain)" ]; then
  echo "Refusing to deploy a dirty worktree. Commit and push the exact source first." >&2
  exit 1
fi
remote_sha=$(git -C "$root" ls-remote origin refs/heads/main | awk 'NR == 1 { print $1 }')
node "$root/scripts/release-identity.mjs" published "$sha" "$remote_sha"

if [ -z "${PREBUILT_IMAGE:-}" ]; then
  az acr build --registry sociobotregistry --image "$tag" --file Dockerfile \
    --build-arg "BUILD_SHA=$sha" --build-arg "GIT_SHA=$sha" --build-arg "SOURCE_COMMIT=$sha" "$root"
else
  tag=${PREBUILT_IMAGE#sociobotregistry.azurecr.io/}
fi

# A full resource PUT makes the state boundary explicit. Keep the custom
# domain binding while replacing the image; do not use deploy-container.sh,
# which intentionally defaults generic apps to maxReplicas: 3.
az rest --method put \
  --uri "$base/providers/Microsoft.App/containerApps/$app?api-version=2024-03-01" \
  --body "$(cat <<EOF
{
  "location":"eastus2",
  "identity":{"type":"UserAssigned","userAssignedIdentities":{"$identity_id":{}}},
  "properties":{
    "managedEnvironmentId":"$environment_id",
    "workloadProfileName":"Consumption",
    "configuration":{
      "activeRevisionsMode":"Single",
      "ingress":{
        "external":true,"targetPort":8080,"transport":"auto","allowInsecure":false,
        "customDomains":[{"name":"integration-changelog-watch.sociobot.in","bindingType":"SniEnabled","certificateId":"$certificate_id"}]
      },
      "registries":[{"server":"sociobotregistry.azurecr.io","identity":"$identity_id"}]
    },
    "template":{
      "terminationGracePeriodSeconds":30,
      "containers":[{
        "name":"app","image":"sociobotregistry.azurecr.io/$tag",
        "resources":{"cpu":0.5,"memory":"1Gi"},"env":[{"name":"PORT","value":"8080"},{"name":"BUILD_SHA","value":"$sha"}],
        "volumeMounts":[{"volumeName":"workspace-data","mountPath":"/data"}]
      }],
      "scale":{"minReplicas":1,"maxReplicas":1},
      "volumes":[{"name":"workspace-data","storageType":"AzureFile","storageName":"integration-changelog-watch-data"}]
    }
  }
}
EOF
)" --output none

deadline=$((SECONDS + 300))
verified=0
while [ "$SECONDS" -lt "$deadline" ]; do
  health=$(curl --fail --silent --show-error --max-time 15 "$site_url/health" 2>/dev/null || true)
  html=$(curl --fail --silent --show-error --max-time 15 "$site_url/" 2>/dev/null || true)
  if printf '%s\n%s' "$health" "$html" | node "$root/scripts/release-identity.mjs" live "$sha" >/dev/null 2>&1; then
    verified=1
    break
  fi
  sleep 5
done

if [ "$verified" -ne 1 ]; then
  printf '%s\n%s' "$health" "$html" | node "$root/scripts/release-identity.mjs" live "$sha"
  exit 1
fi

echo "Deployed sociobotregistry.azurecr.io/$tag with Azure Files /data and one replica. Live identity: $sha"

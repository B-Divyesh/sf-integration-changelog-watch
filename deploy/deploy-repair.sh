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
base="https://management.azure.com/subscriptions/$subscription/resourceGroups/$resource_group"
environment_id="$base/providers/Microsoft.App/managedEnvironments/$environment"
identity_id="$base/providers/Microsoft.ManagedIdentity/userAssignedIdentities/factory-worker-identity"
certificate_id="$environment_id/managedCertificates/cert-integration-changelog-watch"

az acr build --registry sociobotregistry --image "$tag" --file Dockerfile \
  --build-arg "BUILD_SHA=$sha" --build-arg "GIT_SHA=$sha" --build-arg "SOURCE_COMMIT=$sha" "$root"

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
        "resources":{"cpu":0.5,"memory":"1Gi"},"env":[{"name":"PORT","value":"8080"}],
        "volumeMounts":[{"volumeName":"workspace-data","mountPath":"/data"}]
      }],
      "scale":{"minReplicas":1,"maxReplicas":1},
      "volumes":[{"name":"workspace-data","storageType":"AzureFile","storageName":"integration-changelog-watch-data"}]
    }
  }
}
EOF
)" --output none

echo "Deployed sociobotregistry.azurecr.io/$tag with Azure Files /data and one replica."

export type Watch = {id:string|number; vendor:string; url:string; keywords:string; owner:string; version:string; command:string; lastScan?:string}
export type Action = {id:string|number; watchId:string|number; title:string; excerpt:string; matched:string; url:string; owner:string; version:string; command:string; acknowledged:boolean; seenAt:string}

export const sampleWatches: Watch[] = [
 {id:'stripe',vendor:'Stripe',url:'https://docs.stripe.com/changelog/rss.xml',keywords:'breaking,api version,webhook',owner:'Maya · Payments',version:'stripe-node 16.2',command:'pnpm test:stripe'},
 {id:'auth0',vendor:'Auth0',url:'https://auth0.com/changelog/rss',keywords:'deprecation,breaking,token',owner:'Ishan · Identity',version:'auth0-spa-js 2.1',command:'pnpm test:auth'},
 {id:'segment',vendor:'Segment',url:'https://segment.com/docs/release-notes/rss.xml',keywords:'deprecated,destination,breaking',owner:'Nora · Data',version:'analytics-next 1.68',command:'pnpm test:analytics'}
]
export const sampleActions: Action[] = [
 {id:'a1',watchId:'stripe',title:'Stripe retires legacy webhook event format',excerpt:'The legacy event shape stops after the stated migration window. Review signature parsing and event fixtures.',matched:'webhook',url:'https://docs.stripe.com/changelog',owner:'Maya · Payments',version:'stripe-node 16.2',command:'pnpm test:stripe',acknowledged:false,seenAt:'Today'},
 {id:'a2',watchId:'auth0',title:'Auth0 changes refresh token rotation defaults',excerpt:'New tenants use a changed default. Check your explicit configuration before the next environment.',matched:'token',url:'https://auth0.com/changelog',owner:'Ishan · Identity',version:'auth0-spa-js 2.1',command:'pnpm test:auth',acknowledged:true,seenAt:'Yesterday'}
]

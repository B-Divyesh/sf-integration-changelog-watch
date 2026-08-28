import type { Action } from './sample'

export function actionCsv(cards:Action[]) {
  const rows = [['title','owner','matched','command','acknowledged'], ...cards.map(a => [a.title,a.owner,a.matched,a.command,String(a.acknowledged)])]
  return rows.map(r => r.map(x => '"' + x.replaceAll('"','""') + '"').join(',')).join('\n')
}

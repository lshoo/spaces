import './app.css'
import App from './App.svelte'
// import Todolist from './Todolist.svelte'

// const app = new Todolist({
//   target: document.body,
// })
const app = new App({
  target: document.getElementById('app'),
})

export default app

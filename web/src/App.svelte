<script lang="ts">
  import Clock from "./Clock.svelte";
  import Login from "./Login.svelte";
  import Checklist from "./Checklist.svelte";

  import ferrisLogo from '/ferris.png';
  import fermyonLogo from '/fermyon.png';

  let name = "World";
  let color = "red";

  let upper = false;
  $:greeting = `Hello ${name}!`;
  $:casedGreeting = upper ? greeting.toUpperCase() : greeting;
</script>

<main>
  <div>
    <a
      href="https://https://www.rust-lang.org/"
      target="_blank"
      rel="noreferrer"
    >
      <img src={ferrisLogo} class="logo svelte" alt="Ferris Logo" />
    </a>

    <a href="https://www.fermyon.com" target="_blank" rel="noreferrer">
      <img src={fermyonLogo} class="logo" alt="Fermyon Logo" />
    </a>   
    
    <div>
      <label for="name">Name</label>
      <input id="name" bind:value={name} />
    </div>
    
      
    <div>
      <label for="color">Color</label>
      <input id="color" type="color" bind:value={color} />
      <div style="background-color: {color}" class="swatch" />
    </div>
    
    <div>
      <label>
        <input type="checkbox" bind:checked={upper} /> Uppercase
      </label>
    </div>
    
    
    <h2 style="color: {color}">{casedGreeting}</h2>

    <Clock color={color} />

    <h2 class="hero">Travel Packing Checklist</h2>
    <Login />

    <Checklist />

  </div>

  
</main>

<style>
  :global(body) {
    background-color: cornflowerblue;
  }

  .hero {
    --height: 7rem; /* 定义一个css变量 */

    background-color: orange;
    color: white;
    font-size: 4rem;
    height: var(--height);  /* 使用变量 */
    line-height: var(--height);
    margin: 0 0 3rem 0;
    text-align: center;
    vertical-align: middle;
    width: 100vw;
  }

  main {
    color: white;
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    align-items: center;
  }
  
  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: filter 300ms;
  }
  .logo:hover {
    filter: drop-shadow(0 0 2em #646cffaa);
  }
  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00aa);
  }
  ul {
    list-style-type: none;
    margin: auto;
    display: block;
  }
  li {
    margin: 5px;
    padding: 5px;
    float: left;
    min-width: 22%;
    border: 1px solid #ccc;
  }
  h2 {
    color: red;
  }
  .swatch {
    display: inline-block;
    height: 20px;
    width: 20px;
  }
</style>

<script lang="ts">
  import ferrisLogo from '/ferris.png';
  import fermyonLogo from '/fermyon.png';
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
  </div>

  
</main>

<style>
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
</style>

<script>
  // import { Route } from 'tinro'
  import Transactions from './pages/Transactions.svelte'
  // import { url } from '@roxi/routify'
</script>

<style>
  :global(b), :global(h1), :global(h2), :global(h3), :global(h4), :global(h5), :global(h6) {
    font-family: "Overpass";
    font-weight: 600;
  }
  :global(body) {
    margin: 0px 30px;
    background-color: var(--bg-color-2);
  }
  :global(#app) {
    max-width: 1200px;
    margin: auto;
    font-size: 15px;
    font-family: "Inter", sans-serif;
    color: var(--text-color);
  }
  :global(a) {
    color: inherit;
    text-decoration: none;
  }
  :root {
    --bg-color-1: #ffffff;
    --bg-color-2: #f8f9fc;
    --text-color: #444444;
    --shadow: 0px 0px 5px 0px rgba(0,0,0,0.1);
  }
</style>

<template>
  <div class="navbar">
    <a href="/" class="nav-item logo">Kryp</a>
    <a href="/" class="nav-item">Dashboard</a>
    <a href="/wallets" class="nav-item">Wallets</a>
    <a href="/transactions" class="nav-item">Transactions</a>
  </div>
  <!-- <Route path="/transactions"><Transactions/></Route> -->
</template>

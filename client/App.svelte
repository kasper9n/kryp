<script>
  import { Router, navigate, Route, link } from 'svelte-routing'
  import Home from './pages/Home.svelte'
  import Wallets from './pages/Wallets.svelte'
  import Transactions from './transactions/Page.svelte'
  import NotFound from './pages/NotFound.svelte'
  import Navbar from './Navbar.svelte'
  import NavLink from './NavLink.svelte'

  export let url = ''
</script>

<style>
  :root {
    --bg-color-1: #ffffff;
    --bg-color-2: #f8f9fc;
    --text-color: #444444;
    --shadow: 0px 0px 5px 0px rgba(0,0,0,0.1);
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
  :global(b), :global(h1), :global(h2), :global(h3), :global(h4), :global(h5), :global(h6) {
    font-family: "Overpass";
    font-weight: 600;
  }
  :global(a) {
    color: inherit;
    text-decoration: none;
  }
  .navbar {
    display: flex;
    align-items: center;
    max-width: 1200px;
    height: 70px;
    user-select: none;
  }
  .logo {
    font-size: 24px;
  }
  .nav-item {
    padding: 4px 8px;
    cursor: pointer;
  }
  .nav-item:hover {
    opacity: 0.75
  }
  .spacer {
    width: 26px
  }
</style>

<Router url="{url}">
  <div class="navbar">
    <a use:link href="/" class="nav-item logo">Kryp</a>
    <div class="spacer"></div>
    <NavLink to="/">
      <div class="nav-item">Dashboard</div>
    </NavLink>
    <div class="spacer"></div>
    <NavLink to="/wallets">
      <div class="nav-item">Wallets</div>
    </NavLink>
    <div class="spacer"></div>
    <NavLink to="/transactions">
      <div class="nav-item">Transactions</div>
    </NavLink>
    <a use:link href="/idkwhat" class="nav-item">404</a>
  </div>
  <div>
    <Route path="/"><Home/></Route>
    <Route path="/wallets"><Wallets/></Route>
    <Route path="/transactions"><Transactions/></Route>
    <Route><NotFound/></Route>
  </div>
</Router>

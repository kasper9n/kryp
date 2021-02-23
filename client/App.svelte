<script>
  import { Router, navigate, Route, link } from 'svelte-routing'
  import Home from './pages/Home.svelte'
  import Wallets from './pages/Wallets.svelte'
  import Transactions from './pages/Transactions.svelte'
  import Login from './pages/Login.svelte'
  import Logout from './pages/Logout.svelte'
  import Signup from './pages/Signup.svelte'
  import Settings from './pages/Settings.svelte'
  import NotFound from './pages/NotFound.svelte'
  import NavLink from './NavLink.svelte'
  import { Meteor } from 'meteor/meteor'
  import { useTracker } from 'meteor/rdb:svelte-meteor-data'

  export let url = ''

  let userId
  $: userId = useTracker(() => Meteor.user())
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
  .navbar-split {
    margin-left: auto
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
    {#if $userId}
      <div class="spacer"></div>
      <NavLink to="/wallets">
        <div class="nav-item">Wallets</div>
      </NavLink>
      <div class="spacer"></div>
      <NavLink to="/transactions">
        <div class="nav-item">Transactions</div>
      </NavLink>
      <div class="spacer"></div>
      <NavLink to="/logout">
        <div class="nav-item">Log out</div>
      </NavLink>
      <div class="navbar-split"></div>
      <a use:link href="/idkwhat" class="nav-item">404</a>
      <div class="spacer"></div>
      <NavLink to="/settings">
        <div class="nav-item">Settings</div>
      </NavLink>
    {:else}
      <div class="spacer"></div>
      <NavLink to="/login">
        <div class="nav-item">Login</div>
      </NavLink>
      <div class="spacer"></div>
      <NavLink to="/signup">
        <div class="nav-item">Signup</div>
      </NavLink>
    {/if}
  </div>
  <div>
    <Route><NotFound/></Route>
    <Route path="/"><Home/></Route>
    <Route path="/wallets"><Wallets/></Route>
    <Route path="/transactions"><Transactions/></Route>
    <Route path="/logout"><Logout/></Route>
    <Route path="/login"><Login/></Route>
    <Route path="/signup"><Signup/></Route>
    <Route path="/settings"><Settings/></Route>
  </div>
</Router>

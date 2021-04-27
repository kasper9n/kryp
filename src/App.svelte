<script lang="ts">
  // import { Router, navigate, Route, link } from 'svelte-routing'
  import { goto as goTo, path, pattern } from 'svelte-pathfinder'
  import { gun } from './gun'

  // $: gun.get('me').put({
  //   name: 'Mark',
  //   email: 'mark@gunDB.io',
  // })

  let nameInput = ''
  function saveName() {
    gun.get('me').put({
      name: nameInput,
      email: 'example@example.example',
    })
  }
  function deleteName() {
    gun.get('me')
  }

  let user: { name: string; email: string } | null = null
  gun.get('me').on((data, key) => {
    user = data
    console.log('update:', key, data)
  })

  function link(node: HTMLElement) {
    node.onclick = (e) => {
      e.preventDefault()
      const href = node.getAttribute('href') || ''
      goTo(href)
    }
    const unsubscribe = path.subscribe((value) => {
      console.log(value)
    })
    return {
      destroy: () => unsubscribe(),
    }
  }
</script>

<style lang="sass">
  :root
    --bg-color-1: #ffffff
    --bg-color-2: #f8f9fc
    --text-color: #444444
    --shadow: 0px 0px 5px 0px rgba(0,0,0,0.1)
  :global(body)
    margin: 0px 30px
    background-color: var(--bg-color-2)
  main
    max-width: 1200px
    margin: auto
    font-size: 15px
    font-family: "Inter", sans-serif
    color: var(--text-color)
  :global(b), :global(h1), :global(h2), :global(h3), :global(h4), :global(h5), :global(h6)
    font-family: "Overpass"
    font-weight: 600
  :global(a)
    color: inherit
    text-decoration: none
  .navbar
    display: flex
    align-items: center
    max-width: 1200px
    height: 70px
    user-select: none
  .logo
    font-size: 24px
  .nav-item
    padding: 4px 8px
    cursor: pointer
  .navbar-split
    margin-left: auto
  .nav-item:hover
    opacity: 0.75
  .spacer
    width: 26px
</style>

<main>
  <div class="navbar">
    <a use:link href="/" class="nav-item logo">Kryp</a>
    <div class="spacer" />
    <a use:link href="/dashboard" class="nav-item">Dashboard</a>
    <div class="navbar-split" />
  </div>
  <div>
    {#if user}
      Logged in as {user.name}
    {:else}
      Not logged in
    {/if}

    <input bind:value={nameInput} />
    <button on:click={saveName}>Save</button>
    <button on:click={deleteName}>Delete</button>

    {#if publicRoute}
      {#await publicRoute.value}
        Loading...
      {:then component}
        <svelte:component this={component} />
      {/await}
    {/if}
  </div>
</main>

<!-- <Router url="{url}">
  <div class="navbar">
    <a use:link href="/" class="nav-item logo">Kryp</a>
    <div class="spacer"></div>
    <NavLink to="/">
      <div class="nav-item">Dashboard</div>
    </NavLink>
    {#if userId}
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
</Router> -->

<script>
  // import { Route, active } from 'tinro/cmp/index.js'
  import { Router, navigate, Route, link } from 'svelte-routing'
  import Transactions from './pages/Transactions.svelte'
  import Task from './pages/Transactions.svelte'
  import Navbar from './Navbar.svelte'

  export let url = ""

  // document.addEventListener('click', (e) => {
  //   const href = e.target.getAttribute('href')
  //   if (e.target.nodeName === 'A' && href && href.startsWith('/')) {
  //     e.preventDefault()
  //     navigate(href)
  //   }
  // }, true)

  // function isActive(x, y, z) {
  //   // const isActive = href === "/" ? isCurrent : isPartiallyCurrent || isCurrent
  //   console.log(x)

  //   return false
  //   // return isActive
  // }
</script>

<style>
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
    margin-right: 26px;
    cursor: pointer;
  }
  .active {
    color: #00f07c;
  }
</style>

<Router url="{url}">
  <div class="navbar">
    <a use:link href="/" class="nav-item logo">Kryp</a>
    <a use:link href="/" class="nav-item">Dashboard</a>
    <a use:link href="/wallets" class="nav-item">Wallets</a>
    <a use:link href="/transactions" class="nav-item">Transactions</a>
  </div>
  <div>
    <Route path="/">Home</Route>
    <Route path="/wallets">wallettttts</Route>
    <Route path="/transactions">txxxxx</Route>
    <!-- <Route path="blog" component="{Blog}" /> -->
  </div>
</Router>

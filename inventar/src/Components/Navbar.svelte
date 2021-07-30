<script>
  import { logout } from "../lib";
  export let isLoggedIn = false;
  function handleLogout() {
    isLoggedIn = logout()
  }
</script>

<nav class="navbar navbar-light bg-light landing-nav-height">
  <div class="container-fluid">
    <span class="navbar-brand mb-0 fs-3 fw-bold">Inventar</span>
    {#if isLoggedIn}
      <button class="btn btn-danger" on:click={handleLogout}
        ><i class="fas fa-sign-out-alt" /></button
      >
    {/if}
  </div>
</nav>

<style>
  .landing-nav-height {
    height: 10vh;
  }
</style>

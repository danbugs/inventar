<script>
  import Dashboard from "./Dashboard.svelte";
  import AuthForm from "../Components/AuthForm.svelte";
  import Navbar from "../Components/Navbar.svelte";
  import Spinner from "../Components/Spinner.svelte";
  import ErrorView from "../Components/ErrorView.svelte";
  import SloganView from "../Components/SloganView.svelte";
  import Footer from "../Components/Footer.svelte";
  let isLoggedIn =
    sessionStorage.getItem("loggedIn") !== "false" &&
    sessionStorage.getItem("loggedIn") !== null;
</script>

<svelte:head>
  <link
    href="https://cdn.jsdelivr.net/npm/bootstrap@5.0.2/dist/css/bootstrap.min.css"
    rel="stylesheet"
    integrity="sha384-EVSTQN3/azprG1Anm3QDgpJLIm9Nao0Yz1ztcQTwFspd3yD65VohhpuuCOmLASjC"
    crossorigin="anonymous"
  />
  <link
    rel="stylesheet"
    href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/5.15.3/css/all.min.css"
    integrity="sha512-iBBXm8fW90+nuLcSKlbmrPcLa0OT92xO1BIsZ+ywDWZCvqsWgccV3gFoRBv0z+8dLJgyAHIhR35VZc2oM/gI1w=="
    crossorigin="anonymous"
    referrerpolicy="no-referrer"
  />
</svelte:head>

{#await isLoggedIn}
  <Spinner />
{:then}
  <main class={!isLoggedIn ? "landing-background" : ""}>
    <Navbar bind:isLoggedIn />
    <div class="container">
      {#if isLoggedIn}
        <Dashboard />
      {:else}
        <div
          class="row landing-row justify-content-around align-items-md-center"
        >
          <SloganView />
          <AuthForm bind:isLoggedIn />
        </div>
      {/if}
    </div>
    <Footer />
  </main>
{:catch error}
  <main class="landing-background">
    <Navbar isLoggedIn={false} />
    <div class="container">
      <div class="row landing-row justify-content-around align-items-md-center">
        <SloganView />
        <ErrorView errorMsg={error.message} />
      </div>
    </div>
  </main>
{/await}

<style>
  .landing-background {
    background-image: url("https://i.imgur.com/0YGGamV.jpg");
    background-repeat: no-repeat;
  }
</style>

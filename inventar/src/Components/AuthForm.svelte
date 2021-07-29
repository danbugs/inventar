<script>
  import ErrorView from "./ErrorView.svelte";
  import LoginForm from "./AuthForm/LoginForm.svelte";
  import RegistrationForm from "./AuthForm/RegistrationForm.svelte";
  import Spinner from "./Spinner.svelte";

  import { login, register } from "../lib";
  import { LoginUser, RegisterUser } from "../models";
  export let isLoggedIn;
  let isRegistered = true;

  function handleLogin() {
    let formElement = document.getElementById("loginForm");
    let formData = new FormData(formElement);
    let lu = new LoginUser(formData.get("user_name"), formData.get("user_pwd"));
    isLoggedIn = login(lu);
  }

  function handleRegister() {
    let formElement = document.getElementById("registerForm");
    let formData = new FormData(formElement);
    let ru = new RegisterUser(
      formData.get("user_email"),
      formData.get("user_name"),
      formData.get("user_pwd")
    );
    isRegistered = register(ru);
  }
</script>

{#await isRegistered}
  <div class="col-sm-12 col-md-6">
    <Spinner />
  </div>
{:then _}
  <div class="col-sm-12 col-md-6">
    <div class="bg-light m-3 p-3 border rounded-3">
      {#if isRegistered}
        <LoginForm bind:isRegistered on:submit={handleLogin} />
      {:else}
        <RegistrationForm bind:isRegistered on:submit={handleRegister} />
      {/if}
    </div>
  </div>
{:catch error}
    <ErrorView errorMsg={error.message} />
{/await}

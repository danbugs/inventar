<script>
  import { onMount, onDestroy } from "svelte";

  export let isRegistered;

  function verifyUser() {
    document.getElementById("register-btn").removeAttribute("disabled");
  }

  onMount(() => {
    window.verifyUser = verifyUser;
  });

  onDestroy(() => {
    window.verifyUser = null;
  });
</script>

<svelte:head>
  <script src="https://www.google.com/recaptcha/api.js" async defer></script>
</svelte:head>

<div class="h3">REGISTER</div>
<hr />
<form id="registerForm" on:submit|preventDefault>
  <div class="mb-3">
    <label class="form-label" for="user_name">Email:</label><br />
    <input class="form-control" type="text" name="user_email" required />
    <label class="form-label" for="user_name">Username:</label><br />
    <input class="form-control" type="text" name="user_name" required />
    <div class="form-text">
      The username must contain 3 or more characters, its' first character must
      be a alphabetic, and it may contain numbers and some symbols (.$!@-_)
    </div>
    <label class="form-label" for="user_pwd">Password:</label><br />
    <input class="form-control" type="password" name="user_pwd" required />
    <div class="form-text">
      The password must be between 8 to 15 characters, contain at least one
      lowercase letter, one uppercase letter, one numeric digit, and one special
      character.
    </div>
  </div>
  <div class="g-recaptcha" data-sitekey="6LdT3M0bAAAAAJObLYDcRO7N4f4XXBbjBgMmcBRe" data-callback="verifyUser" />
  <input id="register-btn" class="btn btn-primary" type="submit" value="Register" disabled/>
</form>
<hr />
<div class="row justify-content-md-center">
  <div class="col-lg-3">
    <div class="h5">Have an account?</div>
  </div>
  <div class="col-lg-3">
    <button class="btn btn-success" on:click={() => (isRegistered = true)}
      >Login!</button
    >
  </div>
</div>

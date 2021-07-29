<script>
    export let isRegistered;
</script>

<div class="h3">LOGIN</div>
<hr />
<form id="loginForm" on:submit|preventDefault>
  <div class="mb-3">
    <label class="form-label" for="user_name">Username:</label><br />
    <input class="form-control" type="text" name="user_name" required/>
    <label class="form-label" for="user_pwd">Password:</label><br />
    <input class="form-control" type="password" name="user_pwd" required/>
  </div>
  <input class="btn btn-primary" type="submit" value="Login" />
</form>
<hr />
<div class="row justify-content-md-center">
  <div class="col-lg-3">
    <div class="h5">Not registered?</div>
  </div>
  <div class="col-lg-3">
    <button class="btn btn-success" on:click={() => (isRegistered = false)}
      >Sign Up!</button
    >
  </div>
</div>

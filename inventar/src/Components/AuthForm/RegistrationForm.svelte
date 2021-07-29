<script>
    export let isRegistered;
</script>

<div class="h3">REGISTER</div>
<hr />
<form id="registerForm" on:submit|preventDefault>
  <div class="mb-3">
    <label class="form-label" for="user_name">Email:</label><br />
    <input class="form-control" type="text" name="user_email" required/>
    <label class="form-label" for="user_name">Username:</label><br />
    <input class="form-control" type="text" name="user_name" required/>
    <label class="form-label" for="user_pwd">Password:</label><br />
    <input class="form-control" type="password" name="user_pwd" required/>
  </div>
  <input class="btn btn-primary" type="submit" value="Register" />
</form>
<hr />
<div class="row justify-content-md-center">
  <div class="col-lg-3">
    <div class="h5">Have an account?</div>
  </div>
  <div class="col-lg-3">
    <button
      class="btn btn-success"
      on:click={() => (isRegistered = true)}>Login!</button
    >
  </div>
</div>
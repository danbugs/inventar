<script>
  export let errorMsg;
  export let displayTryAgainButton = true;
</script>

<div class="col-md-6">
  <div
    class="fs-3 fw-bold text-danger badge bg-light text-center landing-message-spacing text-uppercase"
  >
    {errorMsg}
  </div>
  <br />
  {#if displayTryAgainButton}
    <button class="btn btn-danger m-1" on:click={() => window.location.reload()}
      >Try again?</button
    >
  {/if}
</div>

<style>
  .landing-message-spacing {
    letter-spacing: 0.25em;
  }
</style>

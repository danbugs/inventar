<script>
  export let isSmall = false;
</script>

<div class="container">
  <div
    class="row {isSmall
      ? 'm-3'
      : 'landing-row'} justify-content-around align-items-center"
  >
    <div class="spinner-border badge bg-dark text-light" role="status">
      <span class="visually-hidden">Loading...</span>
    </div>
  </div>
</div>

<script>
  export let thing;
  export let categories = [];
  let thing_category = categories.find(
    (c) => c.category_id == thing.category_id
  );
</script>

<div class="col-sm-12 col-md-3">
  <div class="card m-3 text-center">
    <div class="card-body">
      <h5 class="card-title">{thing.thing_name}</h5>
      {#if thing.thing_description}
        <p class="card-text">{thing.thing_description}</p>
      {/if}
      <button class="btn btn-danger border" on:click>
        <i class="fas fa-trash-alt" />
      </button>
      {#if thing.category_id}
        <br /><span
          class="badge {!thing_category.category_color
            ? 'text-dark'
            : thing_category.category_color}"
          >{thing_category.category_name}</span
        >
      {/if}
    </div>
  </div>
</div>

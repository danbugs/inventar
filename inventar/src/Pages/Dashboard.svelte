<script>
  import { createThing, readThings, deleteThing, readCategories } from "../lib";
  import Spinner from "../Components/Spinner.svelte";
  import ErrorView from "../Components/ErrorView.svelte";
  import CreateThingForm from "../Components/CreateThingForm.svelte";
  import ThingCard from "../Components/ThingCard.svelte";
  import Tags from "svelte-tags-input";

  let things = [];
  let categories = [];
  let isCreatingNewCategory = false;
  let tags = "";

  let promise = new Promise((resolve, reject) => {
    init(resolve);
  });

  async function init(callback = null) {
    things = await readThings();
    categories = await readCategories();
    tags = await readCategories();
    if (callback) callback();
  }

  async function handleSubmit() {
    things = await createThing();

    if (isCreatingNewCategory) {
      categories = await readCategories();
      isCreatingNewCategory = false;
    }
  }

  function handleTags(event) {
    tags = event.detail.tags;
  }

  function handleDelete(id) {
    things = deleteThing(id);
  }
</script>

{#await promise}
  <Spinner />
{:then _}
  <CreateThingForm
    bind:isCreatingNewCategory
    {categories}
    on:submit={() => (promise = handleSubmit())}
  />
  <div class="row">
      <div class="category-filter">
        <Tags
          on:tags={handleTags}
          {tags}
          placeholder={"Filter categories..."}
          autoComplete={categories}
          autoCompleteKey={"category_name"}
          id="category-filter"
        />
      </div>
    {#await things}
      <Spinner />
    {:then ts}
      {#each ts as thing}
        {#if tags.find((c) => c.category_id == thing.category_id)}
          <ThingCard
            {thing}
            {categories}
            on:click={() => handleDelete(thing.thing_id)}
          />
        {/if}
      {/each}
      {#if ts.length === 0}
        <div class="justify-content-center fs-1 fw-bold">
          Nothing here yet...
        </div>
      {/if}
    {:catch error}
      <ErrorView errorMsg={error.message} />
    {/await}
  </div>
{/await}

<style>
  .category-filter {
    z-index: 1;
  }
</style>

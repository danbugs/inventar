<script>
  import { createThing, readThings, deleteThing, readCategories } from "../lib";
  import Spinner from "../Components/Spinner.svelte";
  import ErrorView from "../Components/ErrorView.svelte";
  import CreateThingForm from "../Components/CreateThingForm.svelte";
  import ThingCard from "../Components/ThingCard.svelte";

  let things;
  let categories;
  let isCreatingNewCategory = false;

  let promise = new Promise((resolve, reject) => {
    init(resolve);
  });

  async function init(callback = null) {
    things = await readThings();
    categories = await readCategories();
    if (callback) callback();
  }

  async function handleSubmit() {
    things = await createThing();

    if (isCreatingNewCategory) {
      categories = await readCategories();
      isCreatingNewCategory = false;
    }
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
    {#await things}
      <Spinner />
    {:then ts}
      {#each ts as thing}
        <ThingCard
          {thing}
          {categories}
          on:click={() => handleDelete(thing.thing_id)}
        />
      {/each}
      {#if ts.length === 0}
        <div class="justify-content-center fs-1 fw-bold">Nothing here yet...</div>
      {/if}
    {:catch error}
      <ErrorView errorMsg={error.message} />
    {/await}
  </div>
{/await}

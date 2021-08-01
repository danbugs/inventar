<script>
  import { colorStyles } from "../constants";

  export let categories = [];
  export let isCreatingNewCategory = false;
  let selectedCategory;

  function checkSelectedCategory() {
    if (selectedCategory === "new") {
      isCreatingNewCategory = true;
    } else {
      isCreatingNewCategory = false;
    }
  }

  function previewFile() {
  const preview = document.querySelector('img');
  const file = document.querySelector('input[type=file]').files[0];
  const reader = new FileReader();

  reader.addEventListener("load", function () {
    preview.src = reader.result;
  }, false);

  if (file) {
    reader.readAsDataURL(file);
  }
}
</script>

<div class="row">
  <div class="mt-3 mb-3 bg-light p-3 border rounded-3">
    <div class="h3 text-uppercase fw-bold text-center">
      Create something new!
    </div>
    <hr />
    <form on:submit|preventDefault>
      <div class="mb-3 form-group row">
        <div class="mt-1 col-sm-12 col-md-2 ">
          <label class="col-form-label" for="thing_img">Thing image:</label>
        </div>
        <div class="mt-1 col-sm-12 col-md-10">
          <input class="form-control" type="file" accept="image/png, image/jpeg" name="thing_img" on:change={previewFile}/>
        </div>
        <div class="mt-1 col-sm-12 col-md-2 ">
          <p>Thing Image Preview:</p>
        </div>
        <div class="justify-content-center mt-1 col-sm-12 col-md-10 ">
          <img id="thing_img_preview" src="" height="200" alt="Upload to preview...">
        </div>
        <div class="mt-1 col-sm-12 col-md-2 ">
          <label class="col-form-label" for="thing_name">Thing name:</label>
        </div>
        <div class="mt-1 col-sm-12 col-md-10">
          <input class="form-control" type="text" name="thing_name" required />
        </div>
        <div class="mt-1 col-sm-12 col-md-2 ">
          <label class="col-form-label" for="thing_description">Thing description:</label>
        </div>
        <div class="mt-1 col-sm-12 col-md-10">
          <textarea class="form-control" rows="4"type="text" name="thing_description"/>
        </div>
        <div class="mt-1 col-sm-12 col-md-2 ">
          <label class="col-form-label" for="category">Thing Category:</label>
        </div>
        <div class="mt-1 col-sm-12 col-md-10">
          <select
            name="category"
            bind:value={selectedCategory}
            class="form-select"
            on:change={checkSelectedCategory}
          >
            <option selected value=""></option>
            {#each categories as category}
              <option value={category.category_id}>{category.category_name}</option>
            {/each}
            <option value="new">Create New...</option>
          </select>
        </div>
        {#if isCreatingNewCategory}
          <div class="mt-1 col-sm-12 col-md-2 ">
            <label class="col-form-label" for="category_name"
              >Category Name:</label
            >
          </div>
          <div class="mt-1 col-sm-12 col-md-10">
            <input
              class="form-control"
              type="text"
              name="category_name"
              required
            />
          </div>
          <div class="mt-1 col-sm-12 col-md-2 ">
            <label class="col-form-label" for="category_color"
              >Category Color:</label
            >
          </div>
          <div class="mt-1 col-sm-12 col-md-10">
            <select
              name="category_color"
              class="form-select"
              required
            >
              {#each colorStyles as colorStyle}
                <option value={colorStyle.color_value}
                  >{colorStyle.color_name}</option
                >
              {/each}
            </select>
          </div>
        {/if}
      </div>
      <input
        class="col-12 btn btn-primary fw-bold text-uppercase"
        type="submit"
        value="Submit"
      />
    </form>
  </div>
</div>

<script lang="ts">
  let postVar;
  let fileVar;

  function submitForm() {
    event.preventDefault();

    const dataArray = new FormData();
    dataArray.append("file_uploaded", fileVar);

    fetch("http://localhost:8000/api/assignments/upload/", {
      method: "POST",
      headers: [["Content-Type", "multipart/form-data"]],
      body: dataArray,
    })
      .then((response) => {
        console.log(response);
        return response.json();
      })
      .catch((error) => {
        console.error("Error:", error);
      });
  }
</script>

<main>
  <!-- Let the user upload a file -->
  <div>
    <form on:submit={submitForm}>
      <input type="file" bind:files={fileVar} />
      <br />
      <input type="submit" />
    </form>
  </div>
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>

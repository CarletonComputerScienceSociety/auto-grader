<script lang=ts>
  let fileVar;

  function submitForm(){
    event.preventDefault();

    const dataArray = new FormData();
    dataArray.append("payload", fileVar);

    fetch("api_url_here", {
      method: "POST",
      headers: [["Content-Type", "multipart/form-data"]],
      body: dataArray
    })
      .then(response => {
        // Successfully uploaded
      })
      .catch(error => {
        // Upload failed
      });
  }
</script>

<div>
  <form on:submit={submitForm}>
    <input 
      type="file" 
      bind:files={fileVar} />
    <br />
    <input type="submit" />
  </form>
</div>
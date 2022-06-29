<script>
  let main_rectangle_background_color = '#2F3133';
  let text_color = "#FFFFFF";
  let button_color = "#1EAEFF"
</script>

<svelte:head>
	<title>Autograder: Home Page</title>
	<meta name="description" content="Landing page for AutoGrader" />
</svelte:head>

<link href='https://fonts.googleapis.com/css?family=Raleway' rel='stylesheet'>
<link href='https://fonts.googleapis.com/css?family=Playfair Display' rel='stylesheet'>

<html lang="en">
  <body>
    <div class="container" style="--theme-color: {main_rectangle_background_color}">
      <div class="row">
        <div class="left-sided-elements" style="--text-color: {text_color}">
          <h1 id="title-text">AutoGrader</h1>
          <h2 id="descriptive-text">Get meaningful and fast grading and feedback on your code!</h2>
          <button class="btn btn-primary" id="login-button" style="--button-color: {button_color}">Login Now!</button>
        </div>
          <img src="https://images.unsplash.com/photo-1619410283995-43d9134e7656?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1470&q=80" alt="code" id="image">
      </div>
    </div>
  </body>
</html>

<style>
  html, body {
    height: 100%;
    margin: 0;
  }

  .container {
    background-color: var(--theme-color);
    height: 100vh;
    overflow: auto;
  }

  .left-sided-elements {
    display: inline-flex;
    flex-direction: column;
    align-items: left;
    float: left;
    width: 20%;
    margin-left: 3%;
    margin-top: 17%;
    color: white;
    gap: 10px;
  }


  #image {
    margin-right: 50px;
    margin-top: 35px;
    float: right;
    width: 65%;
    height: 75%;
  }

  #title-text {
    font-family: "Playfair Display";
    font-size: 96px;
    margin: 0-5px;
    font-weight: bold;
  }

  #descriptive-text {
    font-family: 'Raleway';
    font-size: 32px;
    font-weight: lighter;
  }
  #login-button {
    margin-bottom: 10px;
    font-family: 'Raleway';
    font-size: 25px;
    text-align: left;
    background-color: var(--button-color);
    color: var(--text-color);
    width: 300px;
    height: 60px;
    font-weight: bolder;
  }

</style>
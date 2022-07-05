<script>
  export let title, dueDate, description;
</script>


<h3 class="title">{title}</h3>
<p class="due-date">{dueDate}</p>
<p class="description">{description}</p>


<style>
  * {
    margin-left: 0.5rem;
    width: 50rem;
  }

  .title {
    margin-top: 0.25rem inherit;
    color:rgb(34, 34, 34);
  }

  .due-date {
    color: rgb(217, 79, 79);
    margin-bottom: 0;
  }

  .description {
    margin-top: 0.5rem;
    font-size: 1.2rem;
    color:rgb(43, 42, 42);
  }
</style>
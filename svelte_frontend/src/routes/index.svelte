<script lang="ts">
	import fileSize from 'filesize';
	import FileDrop from 'filedrop-svelte';
	import { filedrop } from 'filedrop-svelte';
	import type { Files, FileDropOptions } from 'filedrop-svelte';
	let files: Files;

	let options = {};

	// On submit, upload the files
    function handleSubmit(e: Event) {
        e.preventDefault();
        
        // Check if there are files to upload
        if (files.accepted.length > 0) {
			var data = new FormData();
			data.append('file', files.accepted[0]);

            // Send the files to localhost:3001/upload
            fetch('http://localhost:3001/upload', {
                method: 'POST',
                body: data
            }).then(res => res.json()).then(data => {
                console.log(data);
            }).catch(err => {
                console.log(err);
            });
        }
    }
</script>

<h1>
	<a href="https://github.com/chanced/filedrop-svelte">FileDrop</a> Component Example
</h1>
<p>
	see <a href="https://svelte.dev/repl/645841f327b8484093f94b84de8a7e64?version=3.41.0"
		>this REPL for the action example.</a
	>
</p>

<FileDrop on:filedrop={(e) => (files = e.detail.files)} />
{#if files}
	<h3>Accepted files</h3>
	<ul>
		{#each files.accepted as file}
			<li>{file.name} - {fileSize(file.size)}</li>
		{/each}
	</ul>
	<h3>Rejected files</h3>
	<ul>
		{#each files.rejected as rejected}
			<li>{rejected.file.name} - {rejected.error.message}</li>
		{/each}
	</ul>
    <!-- Add a submit button -->
    <form on:submit={handleSubmit}>
        <button type="submit">Upload</button>
    </form>
{/if}

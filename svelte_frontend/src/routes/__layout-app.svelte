<!-- This layout is specifically for pages that appear after a user has logged in (back office type pages)-->
<script>
	import { Navbar } from '../components';
</script>

<main>
	<Navbar />
	<slot />
</main>

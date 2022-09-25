<script>
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api';

	let result = '';

	onMount(() => {
		invoke('knock', { message: 'knock knock' }).then((res) => (result = res));
	});
</script>

<div>
	{result}
</div>
<div>
	<ol>
		<ul><a href="/game-of-life">Game of Life</a></ul>
		<ul><a href="/web-midi">Web MIDI</a></ul>
	</ol>
</div>

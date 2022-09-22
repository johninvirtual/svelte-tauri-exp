<script>
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api';

	let pre;

	onMount(() => {
		const renderLoop = () => {
			invoke('render_game_of_life').then((res) => {
				pre.textContent = res;
				requestAnimationFrame(renderLoop);
			});
		};

		renderLoop();
	});

	const handleRestart = () => {
		invoke('restart_game_of_life').then(() => console.log('d'));
	};
</script>

<div>
	<button on:click={handleRestart}>Restart</button>
</div>
<pre bind:this={pre} />

<style>
	pre {
		letter-spacing: -2px;
		line-height: 8px;
	}
</style>

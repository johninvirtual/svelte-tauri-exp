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

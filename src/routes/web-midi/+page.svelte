<script lang="ts">
	import { getIODevices, getMidiAccess } from '$lib/midi/utils';
	import { onMount } from 'svelte';

	let midi: WebMidi.MIDIAccess | null;

	let inputDevice: WebMidi.MIDIInput | null;

	let midiLog: string[] = [];

	const midiListener = (e: WebMidi.MIDIMessageEvent) => {
		midiLog = [e.data.toString(), ...midiLog];
	};

	onMount(() => {
		getMidiAccess().then((midiAccess) => {
			midi = midiAccess;

			if (midi) {
				const { inputDevices, outputDevices } = getIODevices(midi);

				if (inputDevices.length > 0) {
					inputDevice = inputDevices[0];
					inputDevice.addEventListener('midimessage', midiListener);
				}
			}
		});
	});
</script>

{#if midi}
	<p>
		input devices: {midi.inputs.size}
		<br />
		output devices: {midi.outputs.size}
	</p>
{:else}
	unable to connect midi device
{/if}

{#if inputDevice}
	<pre>
        {inputDevice.id}
        {inputDevice.name}
        {inputDevice.state}
    </pre>
{/if}

<div>
	<h6>Logs:</h6>

	<ul>
		{#each midiLog as log}
			<li>
				{log}
			</li>
		{/each}
	</ul>
</div>

<style>
	ul {
		max-height: 200px;
		overflow: scroll;
	}
</style>

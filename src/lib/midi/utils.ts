export const getMidiAccess = async function (): Promise<WebMidi.MIDIAccess | null> {
	let midi: WebMidi.MIDIAccess | null = null;

	function onMIDISuccess(midiAccess: WebMidi.MIDIAccess) {
		console.log('MIDI ready!');
		midi = midiAccess;
	}

	function onMIDIFailure(msg: string) {
		console.error(`Failed to get MIDI access - ${msg}`);
	}

	await navigator.requestMIDIAccess().then(onMIDISuccess, onMIDIFailure);

	return midi;
};

export const getIODevices = function (midiAccess: WebMidi.MIDIAccess) {
	let inputDevices: WebMidi.MIDIInput[] = [];
	let outputDevices: WebMidi.MIDIOutput[] = [];

	for (const entry of midiAccess.inputs) {
		inputDevices.push(entry[1]);
	}

	for (const entry of midiAccess.outputs) {
		outputDevices.push(entry[1]);
	}

	return { inputDevices, outputDevices };
};

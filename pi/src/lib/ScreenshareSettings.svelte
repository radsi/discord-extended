<script lang="ts">
	import { actionSettings } from "@openaction/svelte-pi";

	let streamFocusedWindow = false;

	$: {
		if ($actionSettings.streamFocusedWindow != undefined) {
			streamFocusedWindow = $actionSettings.streamFocusedWindow;
		}
	}

	function handleSave() {
		$actionSettings = {
			...$actionSettings,
			streamFocusedWindow,
		};
	}

	function handleCancel() {
		streamFocusedWindow = $actionSettings.streamFocusedWindow || false;
	}
</script>

<h2 class="mb-3 text-sm font-semibold text-neutral-100">
	Screenshare Settings
</h2>

{#if $actionSettings.error}
	<div class="mb-3 rounded-lg border border-red-700 bg-red-900/30 p-2 text-xs text-red-300">
		<strong class="font-semibold">Error:</strong>
		{$actionSettings.error}
	</div>
{/if}

<div class="mb-3 flex items-center gap-2">
	<input
		id="streamFocusedWindow"
		type="checkbox"
		bind:checked={streamFocusedWindow}
		class="cursor-pointer rounded border border-neutral-600 bg-neutral-700"
	/>
	<label for="streamFocusedWindow" class="cursor-pointer text-xs font-medium text-neutral-200">
		Stream focused window
	</label>
</div>

<div class="flex gap-2">
	<button
		on:click={handleSave}
		class="cursor-pointer rounded-lg border border-neutral-500 bg-neutral-600 px-3 py-1 text-xs text-white hover:bg-neutral-500"
	>
		Save
	</button>
	<button
		on:click={handleCancel}
		class="cursor-pointer rounded-lg border border-neutral-600 bg-neutral-700 px-3 py-1 text-xs text-neutral-300 hover:bg-neutral-600"
	>
		Cancel
	</button>
</div>

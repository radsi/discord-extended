<script lang="ts">
	import { actionSettings } from "@openaction/svelte-pi";

	let editing = false;
	let soundId = "";
	let guildId = "";

	$: {
		if ($actionSettings.soundId != undefined) {
			soundId = $actionSettings.soundId;
		}
		if ($actionSettings.guildId != undefined) {
			guildId = $actionSettings.guildId;
		}
	}

	function handleSave() {
		$actionSettings = {
			...$actionSettings,
			soundId,
			guildId,
		};
		editing = false;
	}

	function handleCancel() {
		soundId = $actionSettings.soundId || "";
		guildId = $actionSettings.guildId || "";
		editing = false;
	}
</script>

<h2 class="mb-3 text-sm font-semibold text-neutral-100">
	Soundboard Action Settings
</h2>

{#if $actionSettings.error}
	<div class="mb-3 rounded-lg border border-red-700 bg-red-900/30 p-2 text-xs text-red-300">
		<strong class="font-semibold">Error:</strong>
		{$actionSettings.error}
	</div>
{:else if $actionSettings.soundId && !editing}
	<div class="mb-3 rounded-lg border border-green-700 bg-green-900/30 p-2 text-xs text-green-300">
		✓ Sound selected: {soundId} (Guild: {guildId || "none"})
	</div>
{/if}

<div class="mb-2 flex items-center gap-2">
	<span class="min-w-22.5 text-xs font-medium text-neutral-200">Sound ID:</span>
	{#if editing}
		<input
			id="soundId"
			type="text"
			bind:value={soundId}
			placeholder="Enter sound ID"
			class="flex-1 rounded-lg border border-neutral-600 bg-neutral-700 px-2 py-1 text-xs text-neutral-100 placeholder-neutral-500 focus:border-neutral-600 focus:ring-1 focus:ring-neutral-600 focus:outline-none"
		/>
	{:else}
		<span class="text-xs text-neutral-300">{soundId || "Not set"}</span>
	{/if}
</div>

<div class="mb-3 flex items-center gap-2">
	<span class="min-w-22.5 text-xs font-medium text-neutral-200">Guild ID:</span>
	{#if editing}
		<input
			id="guildId"
			type="text"
			bind:value={guildId}
			placeholder="Enter guild ID"
			class="flex-1 rounded-lg border border-neutral-600 bg-neutral-700 px-2 py-1 text-xs text-neutral-100 placeholder-neutral-500 focus:border-neutral-600 focus:ring-1 focus:ring-neutral-600 focus:outline-none"
		/>
	{:else}
		<span class="text-xs text-neutral-300">{guildId || "Not set"}</span>
	{/if}
</div>

{#if editing}
	<div class="mb-3 flex gap-2">
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
{:else}
	<button
		on:click={() => (editing = true)}
		class="cursor-pointer rounded-lg border border-neutral-600 bg-neutral-700 px-3 py-1 text-xs text-white hover:bg-neutral-600"
	>
		Edit
	</button>
{/if}

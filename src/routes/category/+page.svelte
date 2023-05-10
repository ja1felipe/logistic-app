<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import { token } from '../../stores/token';

	let code: String;
	let description: String;

	async function handleSubmit() {
		invoke('create_category', {
			category: {
				code,
				description
			},
			token: $token
		})
			.then((res) => {
				console.log(res);
			})
			.catch((err) => {
				console.log(err);
			});
	}
</script>

<form on:submit={handleSubmit}>
	<input placeholder="Code" bind:value={code} type="text" />
	<input placeholder="Descrição" bind:value={description} type="text" />
	<input placeholder="Enviar" type="submit" />
</form>

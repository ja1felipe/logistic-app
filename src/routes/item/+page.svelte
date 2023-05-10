<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import { token } from '../../stores/token';

	let code: string;
	let description: string;
	let category: string;
	let total_qtd: number;

	async function handleSubmit() {
		console.log($token);
		invoke('create_item', {
			item: {
				code,
				description,
				category,
				total_qtd
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
	<input placeholder="Categoria" bind:value={category} type="text" />
	<input placeholder="Quantidade" bind:value={total_qtd} type="number" />
	<input placeholder="Enviar" type="submit" />
</form>

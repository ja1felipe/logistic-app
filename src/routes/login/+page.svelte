<script lang="ts">
	import { invoke } from '@tauri-apps/api';

	let user: String;
	let email: String;
	let password: String;

	let users: String;

	async function handleLogin() {
		invoke('login', {
			user: { email, password }
		})
			.then((res) => {
				user = res as String;
			})
			.catch((err) => {
				console.error(err);
			});
	}

	async function handleGetAll() {
		invoke('get_all_users', {
			token: ''
		})
			.then((res) => {
				users = res as String;
			})
			.catch((err) => {
				console.error(err);
			});
	}
</script>

<form on:submit|preventDefault={handleLogin}>
	<input placeholder="Email" bind:value={email} type="email" />
	<input placeholder="Senha" bind:value={password} type="password" />

	<input type="submit" value="Entrar" placeholder="Entrar" />
</form>
{JSON.stringify(user)}

{#if user}
	<button on:click={handleGetAll}>Opa</button>
	{JSON.stringify(users)}
{/if}

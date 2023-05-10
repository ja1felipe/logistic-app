<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import { token } from '../../stores/token';

	let tokenReturned: string;
	let email: string;
	let password: string;

	let users: string;

	async function handleLogin() {
		invoke('login', {
			user: { email, password }
		})
			.then((res) => {
				tokenReturned = res as string;
				token.set(tokenReturned);
			})
			.catch((err) => {
				console.error(err);
			});
	}

	async function handleGetAll() {
		invoke('get_all_users', {
			tokenReturned
		})
			.then((res) => {
				users = res as string;
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
{JSON.stringify(tokenReturned)}

{#if tokenReturned}
	<button on:click={handleGetAll}>Opa</button>
	{JSON.stringify(users)}
{/if}

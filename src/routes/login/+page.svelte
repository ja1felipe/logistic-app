<script lang="ts">
	import { invoke } from '@tauri-apps/api';

	let user: String;
	let email: String;
	let password: String;

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
</script>

<form on:submit|preventDefault={handleLogin}>
	<input placeholder="Email" bind:value={email} type="email" />
	<input placeholder="Senha" bind:value={password} type="password" />

	<input type="submit" value="Entrar" placeholder="Entrar" />
</form>
{JSON.stringify(user)}

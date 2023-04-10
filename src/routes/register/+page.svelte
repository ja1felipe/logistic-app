<script lang="ts">
	import { invoke } from '@tauri-apps/api';

	let user: String;
	let name: String;
	let email: String;
	let password: String;

	async function handleRegister() {
		invoke('create_user', {
			newUser: { email, name, password }
		})
			.then((res) => {
				user = res as String;
			})
			.catch((err) => {
				console.error(err);
			});
	}
</script>

<form on:submit|preventDefault={handleRegister}>
	<input placeholder="Nome" bind:value={name} />
	<input placeholder="Email" bind:value={email} type="email" />
	<input placeholder="Senha" bind:value={password} type="password" />

	<input type="submit" value="Criar conta" placeholder="Criar conta" />
</form>
{JSON.stringify(user)}

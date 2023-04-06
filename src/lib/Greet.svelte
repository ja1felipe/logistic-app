<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let name = "";
  let users: Teste[] = [];
  type Teste = {
    id: number,
    name: string,
    email: string,
    password: string,
  }
  async function greet(){
      users = await invoke("get_all_users");
  }
</script>

<div>
  <div class="row">
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button on:click={greet}>
      Greet
    </button>

    <ul>
      {#each users as user}
        <li>
          {user.name}
          {user.email}
        </li>
      {/each}
    </ul>
  </div>
</div>
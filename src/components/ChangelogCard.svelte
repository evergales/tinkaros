<script lang="ts">
    export let log: { version: string | undefined, description: string | undefined };
</script>

<main>
    <div id="card" class="container">
        <h3 style="margin-bottom: .2rem; text-align: center;">{log.version}</h3>
        <p style="font-size: .8rem;">{log.description}</p>
    </div>
</main>

<style>
#card {
  position: relative;
  margin: .5rem;
  line-height: 1rem;
  padding: .5rem;
  overflow: hidden;
}
</style>
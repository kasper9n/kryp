<script lang="ts">
  import { event } from '@tauri-apps/api'
  import { onDestroy } from 'svelte'

  export let extensions: string[] = []
  /** Allow files to be dropped anywhere */
  export let anywhere = false
  export let handleFiles: (files: string[]) => void = () => {
    // noop
  }
  export let handleOneFile: (file: string) => void = () => {
    // noop
  }

  function getValidPaths(paths: string[]) {
    let validPaths = []
    for (const path of paths) {
      for (const ext of extensions) {
        if (path.endsWith('.' + ext)) {
          validPaths.push(path)
          break
        }
      }
    }
    return validPaths
  }

  let hovering = false
  function enter() {
    hovering = true
  }
  function leave() {
    hovering = false
  }

  let files: string[] = []

  const fileDropHover = event.listen('tauri://file-drop-hover', (e) => {
    files = getValidPaths(e.payload as string[])
  })
  onDestroy(async () => {
    const unlisten = await fileDropHover
    unlisten()
  })

  const fileDrop = event.listen('tauri://file-drop', (e) => {
    files = getValidPaths(e.payload as string[])
    if (anywhere || hovering) {
      handleFiles(files)
      if (files.length === 1) {
        handleOneFile(files[0])
      }
    }
  })
  onDestroy(async () => {
    const unlisten = await fileDrop
    unlisten()
  })

  const fileDropCancelled = event.listen('tauri://file-drop-cancelled', () => {
    files = []
  })
  onDestroy(async () => {
    const unlisten = await fileDropCancelled
    unlisten()
  })
</script>

<div class="dropzone" on:mouseenter={enter} on:mouseleave={leave}>
  <slot {files} />
</div>

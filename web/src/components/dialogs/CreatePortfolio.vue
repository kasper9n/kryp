<template lang='pug'>
Dialog(ref='dialog' title='Create portfolio')
  TextBox.textbox(compact ref='input' placeholder='Name' v-model='name')
  template(v-slot:buttons)
    Button(medium @click='close()') Cancel
    Button(@click='close(); $pocket.createPortfolio(name)') Create
</template>

<script>
import Dialog from '@/components/Dialog.vue'
import Button from '@/components/Button.vue'
import TextBox from '@/components/TextBox.vue'

export default {
  components: {
    Dialog,
    Button,
    TextBox,
  },
  props: {
    title: String,
  },
  data () {
    return {
      name: '',
    }
  },
  methods: {
    close () {
      this.$refs.dialog.close()
    },
    open () {
      this.$refs.dialog.open()
      this.$refs.input.focus()
    },
  },
}
</script>

<style lang='sass' scoped>
.textbox
  width: 100%
</style>

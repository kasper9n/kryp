import Gun from 'gun/gun'
interface GunState {
  me: { name: string; email: string }
}
// export const gun = Gun<GunState>(['http://localhost:8765/gun', 'https://kryp-gun-relay.herokuapp.com/gun'])
export const gun = Gun<GunState>(['https://kryp-gun-relay.herokuapp.com/gun'])
// export const gun = new Gun<GunState>()

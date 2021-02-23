import { Meteor } from 'meteor/meteor'
import isX from './is'
export const is = isX

export function mError(code, msg, details?) {
  return new Meteor.Error(code, msg, details)
}

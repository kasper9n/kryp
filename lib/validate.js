const is = {
  required: { name: 'required' },
  min: function(length) {
    return { name: 'min', length: length }
  },

  object: { name: 'object' },

  string: { name: 'string' },
  email: { name: 'validEmail' },
  numeric: { name: 'numeric' },

}

const emailRegex = new RegExp(/^\S+@\S+\.\S+$/)
function isNumeric(str, options) {
  return new RegExp('^[-]?([0-9]*[.])?[0-9]+$').test(str)
}

function hasOwnProp(...args) {
  return Object.prototype.hasOwnProperty.call(...args)
}

function rulesListToObject(rulesList) {
  const rules = {}
  for (const rule of rulesList) {
    rules[rule.name] = rule
  }
  return rules
}

function validate(schema, input) {
  const result = {}
  let errors = {}
  for (const key of Object.keys(schema)) {
    const rules = rulesListToObject(schema[key])
    const value = input[key]

    // required
    if (!hasOwnProp(input, key) || value === '') {
      if (rules.required) errors[key] = 'required'

    // object
    } else if (rules.object) {
      result[key] = value
      if (value.constructor !== Object) errors[key] = 'object'

    // string
    } else if (rules.string) {
      result[key] = value

      // string
      if (typeof value !== 'string') {
        errors[key] = 'string'

      // email
      } else if (rules.validEmail) {
        if (!emailRegex.test(value)) errors[key] = 'validEmail'

      // numeric
      } else if (rules.numeric) {
        if (value === '') errors[key] = 'numeric'
      }
    }

  }
  if (Object.keys(errors).length === 0) errors = null
  return { value: result, errors: errors }
}

module.exports = { validate, is }

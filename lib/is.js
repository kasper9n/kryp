const is = {
  required: { name: 'required' },
  min: function(length) {
    return { name: 'min', length: length }
  },
  max: function(length) {
    return { name: 'max', length: length }
  },

  object: { name: 'object' },

  string: { name: 'string' },
  email: { name: 'email' },
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

is.valid = function(schema, input) {
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
      }

      // min
      if (rules.min) {
        if (value.length < rules.min.length) errors[key] = 'min'
      }

      // max
      if (rules.max) {
        if (value.length > rules.max.length) errors[key] = 'max'
      }

      // email
      if (rules.email) {
        if (!emailRegex.test(value)) errors[key] = 'email'
      }

      // numeric
      if (rules.numeric) {
        if (value === '') errors[key] = 'numeric'
      }
    }

  }
  if (Object.keys(errors).length === 0) errors = null
  return { value: result, errors: errors }
}

module.exports = is

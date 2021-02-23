const emailRegex = new RegExp(/^\S+@\S+\.\S+$/)
function isNumeric(str: string) {
  return new RegExp('^[-]?([0-9]*[.])?[0-9]+$').test(str)
}

function hasOwnProp(arg1, arg2) {
  return Object.prototype.hasOwnProperty.call(arg1, arg2)
}

type Rule = {
  name: string
  length?: number
}

function rulesListToObject(rulesList: Rule[]) {
  const rules: { [key: string]: Rule } = {}
  for (const rule of rulesList) {
    rules[rule.name] = rule
  }
  return rules
}

function makeRule(rule: Rule): Rule {
  return rule
}

export default {
  optional: { name: 'optional' },
  min: function(length): Rule {
    return { name: 'min', length: length }
  },
  max: function(length): Rule {
    return { name: 'max', length: length }
  },

  object: { name: 'object' },

  string: { name: 'string' },
  email: { name: 'email' },
  numeric: { name: 'numeric' },
  integer: { name: 'integer' },
  valid: function(schema, input) {
    const result = {}
    let errors = {}
    for (const key of Object.keys(schema)) {
      const rules = rulesListToObject(schema[key])
      const value = input[key]

      // required
      if (!hasOwnProp(input, key) || value === '') {
        if (!rules.optional) errors[key] = 'required'

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
          if (!isNumeric(value)) errors[key] = 'numeric'
        }

        // integer
      } else if (rules.integer) {
        result[key] = value
        if (!Number.isSafeInteger(value)) errors[key] = 'integer'
      }

    }
    if (Object.keys(errors).length === 0) errors = null
    return { value: result, errors: errors }
  },
}

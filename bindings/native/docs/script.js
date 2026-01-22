class ToggleableElement {
  constructor(elem) {
    this.enabled = true
    this.elem = elem
  }

  enable() {
    this.enabled = true
    this.elem.style.display = 'block'
  }

  disable() {
    this.enabled = false
    this.elem.style.display = 'none'
  }

  resolve(text) {
    if (this.elem.innerHTML.toLowerCase().includes(text)) {
      this.enable()
    } else {
      this.disable()
    }
  }
}

class ToggleableGroup {
  constructor(elem) {
    this.titleElement = new ToggleableElement(elem)
    this.elems = []
    this.enabled = true
  }

  setGroupElement(elem) {
    this.groupElement = new ToggleableElement(elem)
  }

  push(elem) {
    this.elems.push(new ToggleableElement(elem))
  }

  resolve(search) {
    this.elems.forEach(elem => elem.resolve(search))

    if (this.elems.some(elem => elem.enabled)) {
      this.titleElement.enable()
      this.groupElement.enable()
      this.enabled = true
    } else {
      this.titleElement.disable()
      this.groupElement.disable()
      this.enabled = false
    }
  }
}

class APIsWrapper extends ToggleableElement {
  constructor(elem) {
    super(elem)
    this.topElem = document.getElementById('top')
  }

  enable() {
    this.topElem.className = 'top-enabled'
    super.enable()
  }

  disable() {
    this.topElem.removeAttribute('class')
    super.disable()
  }
}

let docContent
let APIs
let APINotFound

let mobile = null
let previousMarkedElem = null
let searchIsEmpty = true
const API = []
const elementsToScrollInto = []

function resolveSearch() {
  const search = document.querySelector('input').value.toLowerCase()
  searchIsEmpty = search.length === 0

  if (mobile) {
    if (searchIsEmpty) {
      docContent.enable()
      APIs.disable()
      return
    }

    docContent.disable()
    APIs.enable()
  }

  API.forEach(APIGroup => APIGroup.resolve(search))

  if (API.some(APIGroup => APIGroup.enabled)) {
    APINotFound.disable()
  } else {
    APINotFound.enable()
  }
}

function resolveBoundaries() {
  const previous = mobile

  mobile = window.innerWidth < 768

  if (previous !== mobile) {
    if (mobile) {
      if (searchIsEmpty) {
        APIs.disable()
      } else {
        docContent.disable()
      }
    } else {
      APIs.enable()
      docContent.enable()

      if (previousMarkedElem) {
        previousMarkedElem.style.fontWeight = 'bold'
        elementsToScrollInto.push(previousMarkedElem)
        previousMarkedElem.scrollIntoView()
      }
    }
  }
}

function markAPIAsSelected(name) {
  previousMarkedElem = [...document.querySelectorAll('#api')].find(
    x => x.innerHTML === name
  )
  previousMarkedElem.style.fontWeight = 'bold'
  elementsToScrollInto.push(previousMarkedElem)
  previousMarkedElem.scrollIntoView()
}

window.addEventListener('load', () => {
  const headerTitle = document.querySelector('div.header .title')
  const path = new URL(window.location).pathname
  const structMatch = path.match(/\/struct(\w+)(\.html)?\/?$/)

  docContent = new ToggleableElement(document.getElementById('doc-content'))
  APIs = new APIsWrapper(document.getElementById('apis'))
  APINotFound = new ToggleableElement(document.getElementById('api-not-found'))
  APINotFound.disable()

  for (const APIElement of document.getElementById('apis').children) {
    switch (APIElement.id) {
      case 'apitype': {
        API.push(new ToggleableGroup(APIElement))

        break
      }

      case 'apilist': {
        const currentAPIIndex = API.length - 1

        API[currentAPIIndex].setGroupElement(APIElement)

        for (const APIElementChild of APIElement.children) {
          API[currentAPIIndex].push(APIElementChild)
        }
      }
    }
  }

  resolveBoundaries()
  window.addEventListener('resize', resolveBoundaries)

  if (/\/decancer_8h(\.html)?$/.test(new URL(window.location).pathname)) {
    const hash = (window.location.hash || '').replace(/^#/, '')

    if (hash.length) {
      let hashFlag = false
      let foundMatchingHash = false
      let previousChild

      for (const child of document.querySelector('.contents').children) {
        if (child.id === hash) {
          previousChild = child
          hashFlag = true
        } else if (hashFlag) {
          foundMatchingHash = true
          markAPIAsSelected(
            [...child.childNodes]
              .find(x => x.nodeType === Node.TEXT_NODE)
              .nodeValue.replace(/\(\)?/, '')
          )
          elementsToScrollInto.push(previousChild)

          break
        }
      }

      if (!foundMatchingHash) {
        const matchingElement = [
          ...document.querySelectorAll('.contents table tbody tr')
        ].find(x => x.id === `r_${hash}`)

        if (matchingElement) {
          markAPIAsSelected(matchingElement.children[1].children[0].innerHTML)
          elementsToScrollInto.push(matchingElement)
        }
      }
    }

    for (const link of document.querySelectorAll('.memItemRight a')) {
      link.style.fontWeight = 'bold'
    }

    for (const returnDoc of document.querySelectorAll('.return dd')) {
      returnDoc.innerHTML = returnDoc.innerHTML.replace(/^(const )?\w+\*? /, '')
    }

    document.querySelector('.contents p').remove()

    for (const textBlock of document.querySelectorAll('.contents .textblock')) {
      textBlock.remove()
    }

    document.querySelector('.contents p a').remove()

    headerTitle.remove()
  } else if (structMatch) {
    const p = [...document.querySelectorAll('.contents p')]
    const structName = structMatch[1].replaceAll('__', '_')

    markAPIAsSelected(structName)

    p[0].remove()
    p[1].remove()

    const stfu = [...document.querySelector('.contents').childNodes].find(
      n =>
        n.nodeType === Node.TEXT_NODE &&
        n.nodeValue ===
          'The documentation for this struct was generated from the following file:'
    )

    stfu.remove()

    document.querySelector('.contents ul:last-child').remove()

    let childFlag = false

    for (const child of [...document.querySelector('.contents').children]) {
      if (
        child.className === 'groupheader' &&
        child.innerHTML === 'Detailed Description'
      ) {
        childFlag = true
        child.remove()
      } else if (childFlag && child.className === 'textblock') {
        const currentContents = document.querySelector('.contents')

        currentContents.insertBefore(child, currentContents.firstChild)

        break
      }
    }

    headerTitle.innerHTML = structName
    headerTitle.style.visibility = 'visible'
    elementsToScrollInto.push(headerTitle)
  } else {
    headerTitle.remove()
  }

  for (const since of document.querySelectorAll('.since dd')) {
    const version = since.innerHTML.trim()

    since.innerHTML = `<a href="https://github.com/null8626/decancer/releases/tag/v${version}">v${version}</a>`
  }

  try {
    document.querySelector('#doc-content #MSearchSelectWindow').remove()
  } catch {
    /* empty */
  }

  try {
    document.querySelector('#doc-content #MSearchResultsWindow').remove()
  } catch {
    /* empty */
  }

  for (const stfu of [...document.querySelectorAll('.summary')]) {
    stfu.remove()
  }

  document.addEventListener('click', event => {
    const elem = event.target || event.srcElement

    if (elem && elem.id === 'api') {
      if (previousMarkedElem) {
        previousMarkedElem.style.fontWeight = 'normal'
      }

      previousMarkedElem = elem
      elem.style.fontWeight = 'bold'
      elem.scrollIntoView()

      if (mobile) {
        APIs.disable()
        docContent.enable()
      }
    }
  })

  for (const memname of document.querySelectorAll('td.memname')) {
    memname.innerHTML = memname.innerHTML.replace(/^DECANCER_EXPORT /, '')
  }

  const input = document.querySelector('input')

  window.addEventListener('keydown', event => {
    if (event.ctrlKey && event.key.toLowerCase() === 'f') {
      input.focus()
      event.preventDefault()
    }
  })

  const search = document.querySelector('input')

  search.removeAttribute('onfocus')
  search.removeAttribute('onblur')
  search.setAttribute('onkeyup', 'resolveSearch(event)')

  document.querySelector('html').style.visibility = 'visible'

  let elem

  while ((elem = elementsToScrollInto.shift())) {
    elem.scrollIntoView()
  }
})

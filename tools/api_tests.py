# Just tool to help finding what is implemented and what is missing

import os
import glob
import xml.etree.ElementTree
import sys


LIBVIRT_API_FILE = "/usr/share/libvirt/api/libvirt-api.xml"
MY_PATH = os.path.dirname(os.path.realpath(__file__))
SRC_PATH = MY_PATH + "/../src"


def get_api_symbols(doc):
    funcs = doc.iter('function')
    macros = doc.iter('macro')
    enums = doc.iter('enum')
    return funcs, macros, enums


def get_sources():
    return glob.glob(SRC_PATH + "/*.rs")


def match(el, content):
    return content.find(el) >= 0


def main():
    filter_by = ""
    if len(sys.argv) > 1:
        filter_by = sys.argv[1]

    doc = xml.etree.ElementTree.parse(LIBVIRT_API_FILE).getroot()

    implemented = set([])
    missing = set([])
    for el in doc.iter('function'):

        if el.get('name').startswith(filter_by):  # What I'm looking for

            status = False
            for source in get_sources():
                f = open(source)
                if match(el.get('name'), f.read()):
                    status = True
                    break
            if status:
                implemented.add(el)
            else:
                missing.add(el)

    print("missing: %s, implemented: %s" % (len(missing), len(implemented)))
    print("missing:")
    for x in missing:
        print(x.attrib)
    # print "implemented:"
    # for x in implemented:
    #     print x.attrib


if __name__ == '__main__':
    main()

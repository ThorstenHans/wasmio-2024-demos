from plugin import Plugin
class Plugin(Plugin):
    def transform(self, input):
        return str.lower(input)
project('raytrace', 'cpp',
  version : '0.1',
  default_options : ['warning_level=3'])

inc = include_directories('include')
fmt_dep = dependency('fmt')
subdir('src')


exe = executable(
  'raytrace',
  'main.cxx',
  include_directories: inc,
  dependencies: [fmt_dep],
  link_with: [src_lib])

test('basic', exe)

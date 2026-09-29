import importlib.util
from pathlib import Path
p=Path(__file__).with_name('import_toril_gis.py')
s=importlib.util.spec_from_file_location('imp',p);m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
assert m.bbox_coords([[-70,30],[-69,31]]) == (-70,30,-69,31)
assert m.intersects((-70,30,-69,31),(-82,20,-64,55))
assert not m.intersects((-10,0,-9,1),(-82,20,-64,55))
fc={'type':'FeatureCollection','features':[{'type':'Feature','geometry':{'type':'Point','coordinates':[-70,30]},'properties':{}},{'type':'Feature','geometry':{'type':'Point','coordinates':[10,10]},'properties':{}}]}
assert len(m.clipped(fc,[-82,20,-64,55])['features'])==1
print('TORIL_IMPORTER_TEST: PASS')
